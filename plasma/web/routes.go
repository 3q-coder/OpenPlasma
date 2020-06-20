package web

func initializeRoutes() {

	Router.GET("/", showIndexPage)

	Router.GET("/transfer", showTransferPage)
	Router.POST("/transfer", createTransfer)

	Router.GET("/withdraw", showWithdrawPage)
	Router.POST("/withdraw", createWithdraw)

	Router.GET("/history/:user_id", getUserHistory)
}
