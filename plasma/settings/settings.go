package settings

import (
	"math/big"
	"os"
	"strconv"
	"time"

	"github.com/joho/godotenv"
)

var (
	Debug bool
	// db
	DBDialect string
	DBHost    string
	DBName    string
	DBPort    string
	DBUser    string
	DBPw      string
	// blockchain
	EthProvider string
	// listener
	NumBlocksToConfirm      uint64
	ListenerSleepTime       time.Duration
	StartBlockListener      uint64
	StartBlockListenerHash  string
	ResendTime              int64
	GasPriceCoef            *big.Int
	ResendGasPriceCoef      *big.Int
	GasPriceResendThreshold *big.Int
	GasLimit                uint64
)

func Init(dotenvFileName string) error {
	if dotenvFileName != "" {
		err := godotenv.Load(dotenvFileName)
		if err != nil {
			return err
		}
	}

	Debug = os.Getenv("DEBUG") == "True"

	// data base settings
	DBDialect = getenv("DB_DIALECT", "sqlite")
	DBHost = os.Getenv("DB_HOST")
	DBName = getenv("DB_NAME", "local.db")
	DBPort = os.Getenv("DB_PORT")
	DBUser = os.Getenv("DB_USER")
	DBPw = os.Getenv("DB_PW")

	// blockchain settings
	EthProvider = os.Getenv("ETH_PROVIDER")

	// listener settings
	ListenerSleepTime = 5 * time.Second
	NumBlocksToConfirm, _ = strconv.ParseUint(os.Getenv("NUM_BLOCKS_TO_CONFIRM"), 10, 64)
	StartBlockListener, _ = strconv.ParseUint(os.Getenv("START_BLOCK_LISTENER"), 10, 64)
	StartBlockListenerHash = os.Getenv("START_BLOCK_HASH")
	ResendTime, _ = strconv.ParseInt(os.Getenv("RESEND_TIME"), 10, 64)
	GasLimit = uint64(150000)
	GasPriceResendThreshold = big.NewInt(120) // in percent

	return nil
}

func getenv(key, fallback string) string {
	value := os.Getenv(key)
	if len(value) == 0 {
		return fallback
	}
	return value
}
