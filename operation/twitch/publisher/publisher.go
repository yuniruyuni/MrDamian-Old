package publisher

import (
	"github.com/yuniruyuni/mrdamian/config"
	"github.com/yuniruyuni/mrdamian/model"
	"github.com/yuniruyuni/mrdamian/operation/pipeline"
)

type Publisher struct {
	id      string
	OAuth   string
	Channel string
	Bot     string
}

func Constructor() pipeline.Constructor {
	return pipeline.Constructor{
		Kind:  "TwitchPublisher",
		Label: "Twitch Publisher",
		Gen: func(id string, c *config.Config) pipeline.Component {
			return NewPublisher(id, c.Bot, c.Channel, c.Token)
		},
	}
}

func NewPublisher(id string, bot string, channel string, oauth string) *Publisher {
	return &Publisher{
		id:      id,
		OAuth:   oauth,
		Channel: channel,
		Bot:     bot,
	}
}

func (p *Publisher) ID() string {
	return p.id
}

func (p *Publisher) Kind() string {
	return "TwitchPublisher"
}

func (p *Publisher) Label() string {
	return "Twitch Publisher"
}

func (p *Publisher) Inputs() model.InputPorts {
	return model.InputPorts{
		model.InputPort{
			ID: model.InputPortID{
				Parent: p.id,
				Name:   "message",
			},
			PropertyNames: model.PropertyNames{
				"from_broadcaster_user_login",
				"from_broadcaster_user_id",
				"viewers",
			},
		},
	}
}

func (p *Publisher) Outputs() model.OutputPorts {
	return model.OutputPorts{}
}

func (p *Publisher) Run() pipeline.Process {
	return NewPublisherProcess(p)
}
