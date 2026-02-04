package service

import (
	"context"
	"fmt"
	"time"

	"connectrpc.com/connect"
	v1 "github.com/byzantium/vortex-gate/gen/v1"
)

// GatewayServer implements the v1.GatewayService.
type GatewayServer struct{}

// NewGatewayServer creates a new GatewayServer.
func NewGatewayServer() *GatewayServer {
	return &GatewayServer{}
}

// Echo echoes the request message with a timestamp.
func (s *GatewayServer) Echo(
	ctx context.Context,
	req *connect.Request[v1.EchoRequest],
) (*connect.Response[v1.EchoResponse], error) {
	msg := req.Msg.Message
	if msg == "" {
		msg = "Who goes there?"
	}

	res := connect.NewResponse(&v1.EchoResponse{
		Message:   fmt.Sprintf("VortexGate says: %s", msg),
		Timestamp: time.Now().Format(time.RFC3339),
	})

	return res, nil
}
