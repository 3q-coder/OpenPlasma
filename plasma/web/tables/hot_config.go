package tables

import (
	"fmt"

	"github.com/GoAdminGroup/go-admin/context"
	"github.com/GoAdminGroup/go-admin/modules/db"
	form2 "github.com/GoAdminGroup/go-admin/plugins/admin/modules/form"
	"github.com/GoAdminGroup/go-admin/plugins/admin/modules/table"
	"github.com/GoAdminGroup/go-admin/template/types/form"
)

func GetHotConfigTable(ctx *context.Context) (userTable table.Table) {

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

	// info ---------------------------------------------------------

	info := userTable.GetInfo().SetFilterFormLayout(form.LayoutThreeCol)
	info.AddField("ID", "id", db.Int).FieldSortable()
	info.AddField("DepositPeriod", "deposit_period", db.Int)
	info.AddField("TransferPeriod", "transfer_period", db.Int)
	info.AddField("OnchainWithdrawalPeriod", "onchain_withdrawal_period", db.Int)
	info.AddField("OffchainWithdrawalPeriod", "offchain_withdrawal_period", db.Int)
	info.AddField("CreatedAt", "created_at", db.Timestamp)
	info.AddField("UpdatedAt", "updated_at", db.Timestamp)

	info.SetTable("hot_configs").SetTitle("HotConfigs").SetDescription("HotConfigs")

	// form ---------------------------------------------------------

	formList := userTable.GetForm()
	formList.AddField("ID", "id", db.Int, form.Default).FieldNotAllowEdit().FieldNotAllowAdd()
	formList.AddField("DepositPeriod", "deposit_period", db.Int, form.Number)
	formList.AddField("TransferPeriod", "transfer_period", db.Int, form.Number)
	formList.AddField("OnchainWithdrawalPeriod", "onchain_withdrawal_period", db.Int, form.Number)
	formList.AddField("OffchainWithdrawalPeriod", "offchain_withdrawal_period", db.Int, form.Number)
	formList.AddField("UpdatedAt", "updated_at", db.Timestamp, form.Default).FieldNotAllowAdd()
	formList.AddField("CreatedAt", "created_at", db.Timestamp, form.Default).FieldNotAllowAdd()

	formList.SetTable("hot_configs").SetTitle("HotConfigs").SetDescription("HotConfigs")

	formList.SetPostHook(func(values form2.Values) error {
		fmt.Println("userTable.GetForm().PostHook", values)
		return nil
	})

	return
}
