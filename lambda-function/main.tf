provider "aws" {
  region = "eu-west-1"
  alias  = "eu-west-1"

  # Make it faster by skipping something
  skip_get_ec2_platforms      = true
  skip_metadata_api_check     = true
  skip_region_validation      = true
  skip_credentials_validation = true
  skip_requesting_account_id  = true
}

provider "aws" {
  region = "us-west-1"
  alias  = "us-west-1"

  # Make it faster by skipping something
  skip_get_ec2_platforms      = true
  skip_metadata_api_check     = true
  skip_region_validation      = true
  skip_credentials_validation = true
  skip_requesting_account_id  = true
}

provider "aws" {
  region = "us-east-1"
  alias  = "us-east-1"

  # Make it faster by skipping something
  skip_get_ec2_platforms      = true
  skip_metadata_api_check     = true
  skip_region_validation      = true
  skip_credentials_validation = true
  skip_requesting_account_id  = true
}

# Create a new IAM role for the Lambda function
resource "aws_iam_role" "lambda_exec" {
  name = "lambda_exec"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
      }
    ]
  })
}

module "us-west-1" {
  source = "./modules/lambda_function"
  zip_path = "bootstrap.zip"
  iam_role = aws_iam_role.lambda_exec

  providers = {
    aws = aws.us-west-1
  }
}

module "us-east-1" {
  source = "./modules/lambda_function"
  zip_path = "bootstrap.zip"
  iam_role = aws_iam_role.lambda_exec

  providers = {
    aws = aws.us-east-1
  }
}

module "eu-west-1" {
  source = "./modules/lambda_function"
  zip_path = "bootstrap.zip"
  iam_role = aws_iam_role.lambda_exec

  providers = {
    aws = aws.eu-west-1
  }
}
