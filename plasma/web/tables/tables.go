package tables

import "github.com/GoAdminGroup/go-admin/plugins/admin/modules/table"

var Generators = map[string]table.Generator{
	"users":                GetUserTable,
	"deposits":             GetDepositTable,
	"transfers":            GetTransferTable,
	"offchain_withdrawals": GetOffchainWithdrawalTable,
	"hot_configs":          GetHotConfigTable,
}
