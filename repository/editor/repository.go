//go:generate mockgen -package=mock -source=$GOFILE -destination=mock/$GOFILE
package editor

import (
	"github.com/golang/mock/gomock"

	"github.com/yuniruyuni/mrdamian/repository/editor/memory"
	"github.com/yuniruyuni/mrdamian/repository/editor/mock"
)

type Repository interface{}

func New() Repository {
	return memory.New()
}

type Mock = mock.MockRepository

func NewMock(ctrl *gomock.Controller) *Mock {
	return mock.NewMockRepository(ctrl)
}
