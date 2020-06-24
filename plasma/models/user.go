package models

import (
	"github.com/DryginAlexander/OpenPlasma/plasma"
	"github.com/jinzhu/gorm"
)

type User struct {
	gorm.Model
	plasma.User
}

func (s *Storage) IsUsernameAvailable(username string) bool {
	var user User
	if err := s.db.First(&user, "username = ?", username).Error; err != nil {
		return true
	}
	return false
}

func (s *Storage) IsUserValid(username, password string) bool {
	var user User
	if err := s.db.First(&user, "username = ?", username).Error; err != nil {
		return false
	}
	if user.Password != password {
		return false
	}
	return true
}

func (s *Storage) CreateUser(_user *plasma.User) error {
	user := User{User: *_user}

	if err := s.db.Create(&user).Error; err != nil {
		return err
	}
	return nil
}

func (s *Storage) GetUsersCount() int {
	var count int
	s.db.Model(&User{}).Count(&count)
	return count
}

func (s *Storage) UserById(id int) (*plasma.User, error) {
	var user User
	if err := s.db.First(&user, "id = ?", id).Error; err != nil {
		return nil, err
	}
	return &user.User, nil
}

func (s *Storage) UserByAddress(addr string) (*plasma.User, error) {
	var user User
	if err := s.db.First(&user, "address = ?", addr).Error; err != nil {
		return nil, err
	}
	return &user.User, nil
}

func (s *Storage) ReduceBalance(id int, value int) error {
	var user User
	if err := s.db.First(&user, "id = ?", id).Error; err != nil {
		return err
	}
	user.Balance -= value
	return s.db.Save(user).Error
}

func (s *Storage) IncreaseBalance(id int, value int) error {
	var user User
	if err := s.db.First(&user, "id = ?", id).Error; err != nil {
		return err
	}
	user.Balance += value
	return s.db.Save(user).Error
}
