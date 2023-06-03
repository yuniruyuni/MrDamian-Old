//go:generate mockgen -package=mock -source=$GOFILE -destination=mock/$GOFILE
package pipeline

import (
	"github.com/golang/mock/gomock"

	"github.com/yuniruyuni/mrdamian/repository/pipeline/memory"
	"github.com/yuniruyuni/mrdamian/repository/pipeline/mock"
)

type Repository interface{}

func New() Repository {
	return memory.New()
}

type Mock = mock.MockRepository

func NewMock(ctrl *gomock.Controller) *Mock {
	return mock.NewMockRepository(ctrl)
}
