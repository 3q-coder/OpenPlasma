package web

import (
	_ "github.com/GoAdminGroup/go-admin/adapter/gin"
	_ "github.com/GoAdminGroup/go-admin/modules/db/drivers/sqlite"

	"github.com/GoAdminGroup/go-admin/engine"
	"github.com/GoAdminGroup/go-admin/modules/config"
	"github.com/GoAdminGroup/go-admin/modules/language"
	"github.com/GoAdminGroup/go-admin/template"
	"github.com/GoAdminGroup/go-admin/template/chartjs"
	"github.com/GoAdminGroup/themes/adminlte"

	"github.com/DryginAlexander/OpenPlasma/plasma/web/tables"
)

func initializeAdmin() {

	eng := engine.Default()

	// global config
	cfg := config.Config{
		Databases: config.DatabaseList{
			"default": {
				MaxIdleCon: 50,
				MaxOpenCon: 150,
				File:       "./admin.db",
				Driver:     "sqlite",
			},
		},
		UrlPrefix: "admin",
		// STORE is important. And the directory should has permission to write.
		Store: config.Store{
			Path:   "./uploads",
			Prefix: "uploads",
		},
		Language: language.EN,
		// debug mode
		Debug: true,
		// log file absolute path
		InfoLogPath:   "/var/logs/info.log",
		AccessLogPath: "/var/logs/access.log",
		ErrorLogPath:  "/var/logs/error.log",
		ColorScheme:   adminlte.ColorschemeSkinBlack,
	}

	// add component chartjs
	template.AddComp(chartjs.NewChart())

	_ = eng.AddConfig(cfg).
		AddGenerators(tables.Generators).
		Use(Router)

	// dashboard page
	eng.HTMLFile("GET", "/admin", "./plasma/web/templates/hello.tmpl", map[string]interface{}{
		"msg": "Hello world",
	})
}
