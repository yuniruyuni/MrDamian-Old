package repository

import (
	"testing"

	"github.com/golang/mock/gomock"

	"github.com/yuniruyuni/mrdamian/repository/editor"
	"github.com/yuniruyuni/mrdamian/repository/event"
	"github.com/yuniruyuni/mrdamian/repository/pipeline"
)

type Mocks struct {
	Pipeline *pipeline.Mock
	Editor   *editor.Mock
	Event    *event.Mock
}

func NewMocks(t *testing.T) (*Repositories, *Mocks) {
	ctrl := gomock.NewController(t)

	mocks := &Mocks{
		Pipeline: pipeline.NewMock(ctrl),
		Editor:   editor.NewMock(ctrl),
		Event:    event.NewMock(ctrl),
	}
	repos := &Repositories{
		Pipeline: mocks.Pipeline,
		Editor:   mocks.Editor,
		Event:    mocks.Event,
	}

	return repos, mocks
}
