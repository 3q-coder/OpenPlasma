package web

func initializeRoutes() {

	// Handle the index route
	Router.GET("/", showIndexPage)

	Router.GET("/article/view/:article_id", getArticle)
}
