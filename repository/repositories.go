package repository

import (
	"github.com/yuniruyuni/mrdamian/repository/editor"
	"github.com/yuniruyuni/mrdamian/repository/event"
	"github.com/yuniruyuni/mrdamian/repository/pipeline"
)

type Repositories struct {
	Pipeline pipeline.Repository
	Editor   editor.Repository
	Event    event.Repository
}

func New() *Repositories {
	return &Repositories{
		Pipeline: pipeline.New(),
		Editor:   editor.New(),
		Event:    event.New(),
	}
}
