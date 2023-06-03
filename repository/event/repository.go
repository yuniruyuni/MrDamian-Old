//go:generate mockgen -package=mock -source=$GOFILE -destination=mock/$GOFILE
package event

import (
	"github.com/golang/mock/gomock"

	"github.com/yuniruyuni/mrdamian/repository/event/mock"
	"github.com/yuniruyuni/mrdamian/repository/event/wails"
)

type Repository interface{}

func New() Repository {
	return wails.New()
}

type Mock = mock.MockRepository

func NewMock(ctrl *gomock.Controller) *Mock {
	return mock.NewMockRepository(ctrl)
}
