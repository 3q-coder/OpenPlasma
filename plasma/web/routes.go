package web

func initializeRoutes() {

	Router.Use(setUserStatus())

	Router.GET("/", showIndexPage)

	Router.GET("/transfer", ensureLoggedIn(), showTransferPage)
	Router.POST("/transfer", ensureLoggedIn(), createTransfer)

	Router.GET("/withdraw", ensureLoggedIn(), showWithdrawPage)
	Router.POST("/withdraw", ensureLoggedIn(), createWithdraw)

	Router.GET("/history/:user_id", ensureLoggedIn(), getUserHistory)

	userRoutes := Router.Group("/u")
	{
		userRoutes.GET("/register", ensureNotLoggedIn(), showRegistrationPage)
		userRoutes.POST("/register", ensureNotLoggedIn(), register)
		userRoutes.GET("/login", ensureNotLoggedIn(), showLoginPage)
		userRoutes.POST("/login", ensureNotLoggedIn(), performLogin)
		userRoutes.GET("/logout", ensureLoggedIn(), logout)
	}
}
