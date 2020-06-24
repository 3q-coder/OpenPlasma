package models

import (
	"github.com/DryginAlexander/OpenPlasma/plasma"
	"github.com/jinzhu/gorm"
)

type Deposit struct {
	gorm.Model
	plasma.Deposit
}

func (s *Storage) CreateDeposit(_dep *plasma.Deposit) error {
	dep := Deposit{Deposit: *_dep}

	if err := s.db.Create(&dep).Error; err != nil {
		return err
	}
	return nil
}

func (s *Storage) DepositsByUserId(id int) ([]plasma.Deposit, error) {
	deps := []*Deposit{}
	err := s.db.Set("gorm:auto_preload", true).
		Where("account_ID = ?", id).Find(&deps).Error
	var plasma_deps []plasma.Deposit
	for _, dep := range deps {
		plasma_deps = append(plasma_deps, dep.Deposit)
	}
	return plasma_deps, err
}
