package presentation

func (a *App) ListCandidates() Candidates {
	// return a.repos.Pipeline.ListCandidates()
	return Candidates{}
}

//nolint:revive // this method needs to declare unused argument for export frontend the arg name.
func (a *App) CreateComponent(id string, position Position) {
	// comp := a.repos.Pipeline.CreateComponent(id, position)
	// a.repos.editor.AddComponent(comp)
	// a.repos.Event.EmitAll(model.UpdateEditor, "")
}

//nolint:revive // this method needs to declare unused argument for export frontend the arg name.
func (a *App) RemoveComponent(id string) {
	// a.repos.Pipeline.RemoveComponent(id)
	// a.repos.Event.EmitAll(model.UpdateEditor, "")
}
