package publisher

import (
	"fmt"
	"strconv"

	"github.com/nicklaw5/helix/v2"

	"github.com/yuniruyuni/mrdamian/operation/pipeline"
)

type Process struct {
	pipeline.PassiveProcess

	Client    *helix.Client
	Token     string
	ChannelID string
	BotID     string
}

func NewPublisherProcess(p *Publisher) *Process {
	client, err := helix.NewClient(&helix.Options{
		UserAccessToken: p.OAuth,
	})
	if err != nil {
		// TODO: errro handling.
		return nil
	}

	resp, err := client.GetUsers(
		&helix.UsersParams{
			Logins: []string{
				p.Channel,
				p.Bot,
			},
		})
	if err != nil {
		// TODO: error hanlding.
		return nil
	}

	proc := &Process{
		Client:    client,
		Token:     p.OAuth,
		ChannelID: resp.Data.Users[0].ID,
		BotID:     resp.Data.Users[1].ID,
	}
	proc.PassiveProcess.Handler = proc.Handler
	return proc
}

//nolint:funlen // temporary long function for migrating from rust to golang.
func (p *Process) Handler(packet *pipeline.Packet) []*pipeline.Packet {
	// TODO: make error handling.

	if packet.Port != "message" {
		return nil
	}

	msg := packet.Message
	fromBroadcasterUserLogin, ok := msg["from_broadcaster_user_login"]
	if !ok {
		return nil
	}
	fromBroadcasterUserID, ok := msg["from_broadcaster_user_id"]
	if !ok {
		return nil
	}
	viewersStr, ok := msg["viewers"]
	if !ok {
		return nil
	}
	viewers, err := strconv.Atoi(string(viewersStr))
	if err != nil {
		return nil
	}

	login := fromBroadcasterUserLogin

	resp, err := p.Client.GetChannelInformation(&helix.GetChannelInformationParams{
		BroadcasterIDs: []string{string(fromBroadcasterUserID)},
	})
	if err != nil {
		return nil
	}
	if len(resp.Data.Channels) != 1 {
		return nil
	}

	game := resp.Data.Channels[0].GameName

	message := fmt.Sprintf(
		"%sさんから%d名のRAIDを頂きました！今日は「%s」を遊んでいたみたい",
		login,
		viewers,
		game,
	)

	_, err = p.Client.SendChatAnnouncement(&helix.SendChatAnnouncementParams{
		BroadcasterID: p.ChannelID,
		ModeratorID:   p.BotID,
		Message:       message,
	})
	if err != nil {
		return nil
	}

	_, err = p.Client.SendShoutout(&helix.SendShoutoutParams{
		FromBroadcasterID: p.ChannelID,
		ToBroadcasterID:   string(fromBroadcasterUserID),
		ModeratorID:       p.BotID,
	})
	if err != nil {
		return nil
	}

	return nil
}
