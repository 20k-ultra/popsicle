# Create a new Lambda function
resource "aws_lambda_function" "popsicle_profiler" {
  # Define the name of the Lambda function
  function_name = "popsicle-profiler"

  # Specify the runtime for the Lambda function
  runtime = "provided"

  # Specify the handler for the Lambda function
  handler = "main"

  # Upload the ZIP file containing the Rust program
  filename = var.zip_path

  # Define the role that the Lambda function should use
  role = var.iam_role.arn

  # Configure the memory and timeout for the Lambda function
  memory_size = 256
  timeout     = 10

  # Define the lifecycle block to handle errors
  lifecycle {
    create_before_destroy = true

    # Specify the retry behavior for creating the Lambda function
    ignore_changes = [
      # Ignore changes to the function name 
      function_name,
    ]
  }
}

# Attach a policy to the IAM role that allows the Lambda function to write logs
resource "aws_iam_role_policy_attachment" "lambda_logs" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = var.iam_role.name
}

# Create a function URL resource 
resource "aws_lambda_function_url" "latest" {
  function_name      = aws_lambda_function.popsicle_profiler.function_name
  authorization_type = "NONE"
}
