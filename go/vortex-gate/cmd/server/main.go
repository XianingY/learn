package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"golang.org/x/net/http2"
	"golang.org/x/net/http2/h2c"

	"connectrpc.com/vanguard"
	"github.com/byzantium/vortex-gate/gen/v1/v1connect"
	"github.com/byzantium/vortex-gate/internal/middleware"
	"github.com/byzantium/vortex-gate/internal/service"
)

func main() {
	// Configuration
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}
	addr := ":" + port

	// Initialize the service
	gateway := service.NewGatewayServer()

	// Create a Vanguard service
	service := vanguard.NewService(v1connect.NewGatewayServiceHandler(gateway))

	// Create the transcoder
	transcoder, err := vanguard.NewTranscoder([]*vanguard.Service{service})
	if err != nil {
		log.Fatalf("failed to create transcoder: %v", err)
	}

	// Build middleware chain
	// Order: Logger -> Auth -> Transcoder
	handler := middleware.Auth(transcoder)
	handler = middleware.Logger(handler)

	// Create the server
	srv := &http.Server{
		Addr: addr,
		// Use h2c (HTTP/2 Cleartext) to support gRPC calls without TLS locally
		Handler: h2c.NewHandler(handler, &http2.Server{}),
	}

	// Start server in a goroutine
	go func() {
		fmt.Printf("🌀 VortexGate listening on %s\n", addr)
		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("failed to serve: %v", err)
		}
	}()

	// Wait for interrupt signal
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	fmt.Println("\nShutting down server...")

	// The context is used to inform the server it has 5 seconds to finish
	// the request it is currently handling
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err := srv.Shutdown(ctx); err != nil {
		log.Fatalf("Server forced to shutdown: %v", err)
	}

	fmt.Println("Server exiting")
}
