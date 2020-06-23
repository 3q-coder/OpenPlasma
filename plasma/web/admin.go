package web

import (
	_ "github.com/GoAdminGroup/go-admin/adapter/gin"
	"github.com/GoAdminGroup/go-admin/engine"
	"github.com/GoAdminGroup/go-admin/modules/config"
	_ "github.com/GoAdminGroup/go-admin/modules/db/drivers/sqlite"
	"github.com/GoAdminGroup/go-admin/modules/language"
	"github.com/GoAdminGroup/go-admin/template"
	"github.com/GoAdminGroup/go-admin/template/chartjs"
	"github.com/GoAdminGroup/themes/adminlte"
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
		// Databases: config.DatabaseList{
		// 	"default": {
		// 		Host:       "127.0.0.1",
		// 		Port:       "5432",
		// 		User:       "django",
		// 		Pwd:        "jw8s0F4",
		// 		Name:       "django",
		// 		MaxIdleCon: 50,
		// 		MaxOpenCon: 150,
		// 		Driver:     config.DriverPostgresql,
		// 	},
		// },
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
		// AddGenerators(datamodel.Generators).
		// // add generator, first parameter is the url prefix of table when visit.
		// // example:
		// //
		// // "user" => http://localhost:9033/admin/info/user
		// //
		// AddGenerator("user", datamodel.GetUserTable).
		Use(Router)

	// customize your pages
	// eng.HTML("GET", "/admin", datamodel.GetContent)

}
