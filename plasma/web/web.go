package web

import (
	"github.com/gin-gonic/gin"
)

var Router *gin.Engine

func Init() {

	Router = gin.Default()

	// TODO: fix path
	Router.LoadHTMLGlob("../../plasma/web/templates/*")

	initializeRoutes()

	Router.Run()
}
