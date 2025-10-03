#!/usr/bin/env python3
"""
Mock Prediction API Server

Simulates the prediction API endpoint for testing the Ebers application.
Returns random probabilities with realistic response times.

Usage:
    python mock_api_server.py [--port PORT] [--delay SECONDS] [--error-rate RATE]

Examples:
    python mock_api_server.py                    # Run on default port 8000
    python mock_api_server.py --port 3000        # Run on port 3000
    python mock_api_server.py --delay 2          # Add 2 second delay to responses
    python mock_api_server.py --error-rate 0.2   # Return errors 20% of the time
"""

import argparse
import json
import random
import time
from datetime import datetime, timezone
from http.server import HTTPServer, BaseHTTPRequestHandler


class PredictionAPIHandler(BaseHTTPRequestHandler):
    """Handler for prediction API requests"""
    
    # Class variables set by command line args
    response_delay = 0
    error_rate = 0.0
    
    def do_POST(self):
        """Handle POST requests to /api/predict"""
        if self.path != '/api/predict':
            self.send_error(404, "Not Found")
            return
        
        # Read request body
        content_length = int(self.headers.get('Content-Length', 0))
        body = self.rfile.read(content_length)
        
        try:
            request_data = json.loads(body)
            self.log_request_data(request_data)
            
            # Simulate processing delay
            if self.response_delay > 0:
                print(f"[API] Simulating {self.response_delay}s processing delay...")
                time.sleep(self.response_delay)
            
            # Randomly return errors based on error_rate
            if random.random() < self.error_rate:
                self.send_error_response(request_data)
                return
            
            # Generate successful prediction response
            self.send_success_response(request_data)
            
        except json.JSONDecodeError as e:
            self.send_error(400, f"Invalid JSON: {str(e)}")
        except Exception as e:
            print(f"[API] Error processing request: {e}")
            self.send_error(500, f"Internal Server Error: {str(e)}")
    
    def log_request_data(self, data):
        """Log incoming request details"""
        dataset_id = data.get('dataset_id', 'unknown')
        row_count = data.get('row_count', 0)
        port = data.get('metadata', {}).get('port', 'unknown')
        
        print(f"\n[API] Received prediction request:")
        print(f"  Dataset ID: {dataset_id}")
        print(f"  Row Count: {row_count}")
        print(f"  Port: {port}")
        print(f"  Timestamp: {data.get('timestamp', 'unknown')}")
    
    def send_success_response(self, request_data):
        """Send a successful prediction response"""
        # Generate random probability and confidence
        probability = random.uniform(0.65, 0.95)
        confidence = random.uniform(0.85, 0.99)
        
        # Calculate processing time (simulate realistic timing)
        processing_time_ms = random.randint(800, 2500)
        
        response = {
            "success": True,
            "dataset_id": request_data.get('dataset_id', 'unknown'),
            "probability": round(probability, 4),
            "confidence": round(confidence, 4),
            "processed_at": datetime.now(timezone.utc).isoformat(),
            "metadata": {
                "model_version": "1.0.0",
                "processing_time_ms": processing_time_ms
            }
        }
        
        print(f"[API] Returning prediction: {response['probability']:.2%} (confidence: {response['confidence']:.2%})")
        
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(json.dumps(response, indent=2).encode())
    
    def send_error_response(self, request_data):
        """Send an error response"""
        error_types = [
            ("INVALID_DATA", "Dataset contains invalid values", "Row 42 has null value"),
            ("MODEL_ERROR", "Prediction model failed", "Insufficient data quality"),
            ("TIMEOUT", "Processing timeout", "Model took too long to respond"),
            ("VALIDATION_ERROR", "Data validation failed", "Expected at least 100 data points"),
        ]
        
        error_code, message, details = random.choice(error_types)
        
        response = {
            "success": False,
            "error": {
                "code": error_code,
                "message": message,
                "details": details
            }
        }
        
        print(f"[API] Returning error: [{error_code}] {message}")
        
        self.send_response(400)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(json.dumps(response, indent=2).encode())
    
    def do_OPTIONS(self):
        """Handle CORS preflight requests"""
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.end_headers()
    
    def log_message(self, format, *args):
        """Override to customize logging"""
        # Only log errors, not every request
        if args[1] != '200':
            super().log_message(format, *args)


def run_server(port=8000, delay=0, error_rate=0.0):
    """Run the mock API server"""
    # Set class variables for handler
    PredictionAPIHandler.response_delay = delay
    PredictionAPIHandler.error_rate = error_rate
    
    server_address = ('', port)
    httpd = HTTPServer(server_address, PredictionAPIHandler)
    
    print("=" * 60)
    print("Mock Prediction API Server")
    print("=" * 60)
    print(f"Server running at: http://localhost:{port}")
    print(f"Endpoint: http://localhost:{port}/api/predict")
    print(f"Response delay: {delay}s")
    print(f"Error rate: {error_rate:.1%}")
    print("\nConfiguration for .env file:")
    print(f"PREDICTION_API_ENDPOINT=http://localhost:{port}/api/predict")
    print("\nPress Ctrl+C to stop the server")
    print("=" * 60)
    print()
    
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n\n[API] Server stopped by user")
        httpd.shutdown()


def main():
    parser = argparse.ArgumentParser(
        description='Mock Prediction API Server for testing',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s                          Run on default port 8000
  %(prog)s --port 3000              Run on port 3000
  %(prog)s --delay 2                Add 2 second delay to responses
  %(prog)s --error-rate 0.2         Return errors 20%% of the time
  %(prog)s --delay 1 --error-rate 0.1  Combine options
        """
    )
    
    parser.add_argument(
        '--port',
        type=int,
        default=8000,
        help='Port to run the server on (default: 8000)'
    )
    
    parser.add_argument(
        '--delay',
        type=float,
        default=0,
        help='Artificial delay in seconds for each response (default: 0)'
    )
    
    parser.add_argument(
        '--error-rate',
        type=float,
        default=0.0,
        help='Probability of returning an error (0.0-1.0, default: 0.0)'
    )
    
    args = parser.parse_args()
    
    # Validate arguments
    if args.port < 1 or args.port > 65535:
        parser.error("Port must be between 1 and 65535")
    
    if args.delay < 0:
        parser.error("Delay must be non-negative")
    
    if args.error_rate < 0 or args.error_rate > 1:
        parser.error("Error rate must be between 0.0 and 1.0")
    
    run_server(port=args.port, delay=args.delay, error_rate=args.error_rate)


if __name__ == '__main__':
    main()

