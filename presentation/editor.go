package presentation

func (a *App) GetEditor() Editor {
	// return a.repos.Editor.Get()
	return Editor{
		Nodes: Nodes{},
		Edges: Edges{},
	}
}

//nolint:revive // this method needs to declare unused argument for export frontend the arg name.
func (a *App) UpdateEditor(editor Editor) {
	// a.repos.Editor.Update(editor)
}
