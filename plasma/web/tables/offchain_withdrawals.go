package tables

import (
	"fmt"

	"github.com/GoAdminGroup/go-admin/context"
	"github.com/GoAdminGroup/go-admin/modules/db"
	form2 "github.com/GoAdminGroup/go-admin/plugins/admin/modules/form"
	"github.com/GoAdminGroup/go-admin/plugins/admin/modules/table"
	"github.com/GoAdminGroup/go-admin/template/types/form"
)

func GetOffchainWithdrawalTable(ctx *context.Context) (userTable table.Table) {

	userTable = table.NewDefaultTable(table.Config{
		Driver:     db.DriverSqlite,
		CanAdd:     true,
		Editable:   true,
		Deletable:  true,
		Exportable: true,
		Connection: table.DefaultConnectionName,
		PrimaryKey: table.PrimaryKey{
			Type: db.Int,
			Name: table.DefaultPrimaryKeyName,
		},
	})

	info := userTable.GetInfo().SetFilterFormLayout(form.LayoutThreeCol)
	info.AddField("ID", "id", db.Int).FieldSortable()
	info.AddField("UserId", "user_id", db.Int).FieldSortable()
	info.AddField("Value", "value", db.Int).FieldSortable()
	info.AddField("Address", "address", db.Varchar).FieldFilterable()

	info.SetTable("offchain_withdrawals").SetTitle("OffchainWithdrawals").SetDescription("OffchainWithdrawals")

	formList := userTable.GetForm()
	formList.AddField("ID", "id", db.Int, form.Default).FieldNotAllowEdit().FieldNotAllowAdd()
	formList.AddField("UserId", "user_id", db.Int, form.Number)
	formList.AddField("Value", "value", db.Int, form.Number)
	formList.AddField("Address", "address", db.Varchar, form.Text)

	formList.SetTable("offchain_withdrawals").SetTitle("OffchainWithdrawals").SetDescription("OffchainWithdrawals")

	formList.SetPostHook(func(values form2.Values) error {
		fmt.Println("userTable.GetForm().PostHook", values)
		return nil
	})

	return
}
