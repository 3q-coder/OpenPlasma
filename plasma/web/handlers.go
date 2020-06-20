package web

import (
	"net/http"
	"strconv"

	"github.com/DryginAlexander/OpenPlasma/plasma"
	"github.com/gin-gonic/gin"
)

func showIndexPage(c *gin.Context) {
	// TODO add authentification
	c.HTML(
		http.StatusOK,
		"index.html",
		gin.H{
			"title": "Home Page",
		},
	)
}

func showTransferPage(c *gin.Context) {
	// TODO add authentification and show current balance
	c.HTML(
		http.StatusOK,
		"transfer.html",
		gin.H{
			"title": "Transfer Page",
		},
	)
}

func showWithdrawPage(c *gin.Context) {
	// TODO add authentification and show current balance
	c.HTML(
		http.StatusOK,
		"withdraw.html",
		gin.H{
			"title": "Withdraw Page",
		},
	)
}

func createTransfer(c *gin.Context) {
	from := c.PostForm("from")
	// TODO add authentification
	if from == "" {
		c.AbortWithError(http.StatusBadRequest, nil)
	}
	to := c.PostForm("to")
	// TODO add address check
	if to == "" {
		c.AbortWithError(http.StatusBadRequest, nil)
	}
	value, err := strconv.Atoi(c.PostForm("value"))
	// TODO add check that user has enough funds
	if err != nil {
		c.AbortWithError(http.StatusBadRequest, err)
	}

	// TODO check Nonce and signature

	transfer := plasma.Transfer{
		From:      from,
		To:        to,
		Value:     value,
		Nonce:     0,
		Signature: "0x0",
	}

	// TODO properly handle concurent requests
	err = operator.CreateTransfer(transfer)
	if err != nil {
		c.AbortWithError(http.StatusBadRequest, err)
	}
}

func createWithdraw(c *gin.Context) {
	from := c.PostForm("from")
	// TODO add authentification
	if from == "" {
		c.AbortWithError(http.StatusBadRequest, nil)
	}
	to := c.PostForm("to")
	// TODO add address check
	if to == "" {
		c.AbortWithError(http.StatusBadRequest, nil)
	}
	value, err := strconv.Atoi(c.PostForm("value"))
	// TODO add check that user has enough funds
	if err != nil {
		c.AbortWithError(http.StatusBadRequest, err)
	}

	// TODO check Nonce and signature
	withdraw := plasma.OffchainWithdrawal{
		Value:     value,
		Address:   to,
		Nonce:     0,
		Signature: "0x0",
	}

	// TODO properly handle concurent requests
	err = operator.CreateOffchainWithdraw(from, withdraw)
	if err != nil {
		c.AbortWithError(http.StatusBadRequest, err)
	}
}

func getUserHistory(c *gin.Context) {
	userId, err := strconv.Atoi(c.Param("user_id"))
	if err != nil {
		c.AbortWithStatus(http.StatusNotFound)
	}
	// TODO check user exist
	deposits, err := storage.DepositsByUserId(userId)
	if err != nil {
		c.AbortWithError(http.StatusNotFound, err)
	}
	transfers, err := storage.TransfersByUserId(userId)
	if err != nil {
		c.AbortWithError(http.StatusNotFound, err)
	}
	onWithdrawals, err := storage.OnchainWithdrawalsByUserId(userId)
	if err != nil {
		c.AbortWithError(http.StatusNotFound, err)
	}
	offWithdrawals, err := storage.OffchainWithdrawalsByUserId(userId)
	if err != nil {
		c.AbortWithError(http.StatusNotFound, err)
	}

	c.HTML(
		http.StatusOK,
		"history.html",
		gin.H{
			"title":          "History Page",
			"deposits":       deposits,
			"transfers":      transfers,
			"onWithdrawals":  onWithdrawals,
			"offWithdrawals": offWithdrawals,
		},
	)
}
