package web

import (
	"github.com/DryginAlexander/OpenPlasma/plasma"
	"github.com/gin-gonic/gin"
)

var Router *gin.Engine
var operator plasma.Operator
var storage plasma.Storage
var hotConfig plasma.HotConfig

func Init(conf plasma.HotConfig, stor plasma.Storage, oper plasma.Operator) {

	operator = oper
	storage = stor
	hotConfig = conf

	Router = gin.Default()

	// TODO: fix path
	Router.LoadHTMLGlob("./plasma/web/templates/*")

	initializeRoutes()

	initializeAdmin()

	Router.Run()
}
