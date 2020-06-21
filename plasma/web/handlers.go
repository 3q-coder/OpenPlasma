package web

import (
	"math/rand"
	"net/http"
	"strconv"

	"github.com/DryginAlexander/OpenPlasma/plasma"
	"github.com/gin-gonic/gin"
)

func showIndexPage(c *gin.Context) {
	render(c, gin.H{"title": "Home Page"}, "index.html")
}

func showTransferPage(c *gin.Context) {
	// TODO show current balance
	render(c, gin.H{"title": "Transfer Page"}, "transfer.html")
}

func showWithdrawPage(c *gin.Context) {
	// TODO show current balance
	render(c, gin.H{"title": "Withdraw Page"}, "withdraw.html")
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

	render(c, gin.H{"title": "Transfer"}, "submission-successful.html")
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

	render(c, gin.H{"title": "Withdraw"}, "submission-successful.html")
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

	render(
		c,
		gin.H{
			"title":          "History Page",
			"deposits":       deposits,
			"transfers":      transfers,
			"onWithdrawals":  onWithdrawals,
			"offWithdrawals": offWithdrawals,
		},
		"history.html",
	)
}

func generateSessionToken() string {
	// TODO use secure way
	return strconv.FormatInt(rand.Int63(), 16)
}

func showRegistrationPage(c *gin.Context) {
	c.HTML(
		http.StatusOK,
		"register.html",
		gin.H{
			"title": "Register",
		},
	)
}

func register(c *gin.Context) {
	username := c.PostForm("username")
	password := c.PostForm("password")
	address := c.PostForm("address")

	if _, err := operator.RegisterUser(username, password, address); err == nil {
		// If the user is created, set the token in a cookie and log the user in
		token := generateSessionToken()
		c.SetCookie("token", token, 3600, "", "", false, true)
		c.Set("is_logged_in", true)

		render(c, gin.H{"title": "Successful registration & Login"},
			"login-successful.html")

	} else {
		// If the username/password combination is invalid,
		// show the error message on the login page
		c.HTML(
			http.StatusBadRequest,
			"register.html",
			gin.H{
				"ErrorTitle":   "Registration Failed",
				"ErrorMessage": err.Error(),
			},
		)

	}
}

func showLoginPage(c *gin.Context) {
	render(c, gin.H{
		"title": "Login",
	}, "login.html")
}

func performLogin(c *gin.Context) {
	username := c.PostForm("username")
	password := c.PostForm("password")

	if storage.IsUserValid(username, password) {
		token := generateSessionToken()
		c.SetCookie("token", token, 3600, "", "", false, true)
		c.Set("is_logged_in", true)

		render(c, gin.H{
			"title": "Successful Login"}, "login-successful.html")

	} else {
		c.HTML(http.StatusBadRequest, "login.html", gin.H{
			"ErrorTitle":   "Login Failed",
			"ErrorMessage": "Invalid credentials provided"})
	}
}

func logout(c *gin.Context) {
	c.SetCookie("token", "", -1, "", "", false, true)

	c.Redirect(http.StatusTemporaryRedirect, "/")
}

func render(c *gin.Context, data gin.H, templateName string) {
	loggedInInterface, _ := c.Get("is_logged_in")
	data["is_logged_in"] = loggedInInterface.(bool)

	switch c.Request.Header.Get("Accept") {
	case "application/json":
		c.JSON(http.StatusOK, data["payload"])
	case "application/xml":
		c.XML(http.StatusOK, data["payload"])
	default:
		c.HTML(http.StatusOK, templateName, data)
	}
}
