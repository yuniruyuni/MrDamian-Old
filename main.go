package main

import (
	"embed"

	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/options"
	"github.com/wailsapp/wails/v2/pkg/options/assetserver"

	"github.com/yuniruyuni/mrdamian/presentation"
	"github.com/yuniruyuni/mrdamian/repository"
)

//go:embed all:frontend/dist
var assets embed.FS

const (
	Width  = 1024
	Height = 768
)

//nolint:gomnd // constant component values should be in such constant.
var BackgroundColor = options.RGBA{R: 27, G: 38, B: 54, A: 1}

func main() {
	repos := repository.New()
	// Create an instance of the app structure
	app := presentation.NewApp(repos)

	// Create application with options
	err := wails.Run(&options.App{
		Title:  "mrdamian",
		Width:  Width,
		Height: Height,
		AssetServer: &assetserver.Options{
			Assets: assets,
		},
		BackgroundColour: &BackgroundColor,
		OnStartup:        app.Startup,
		Bind: []interface{}{
			app,
		},
	})
	if err != nil {
		println("Error:", err.Error())
	}
}
