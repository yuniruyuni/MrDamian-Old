package presentation

import (
	"context"

	"github.com/yuniruyuni/mrdamian/repository"
)

// App struct.
type App struct {
	ctx   context.Context
	repos *repository.Repositories
}

// NewApp creates a new App application struct.
func NewApp(repos *repository.Repositories) *App {
	return &App{
		repos: repos,
	}
}

// startup is called when the app starts.
// The context is saved so we can call the runtime methods.
func (a *App) Startup(ctx context.Context) {
	a.ctx = ctx
}
