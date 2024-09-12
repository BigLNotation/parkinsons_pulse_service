terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5"
    }
  }

  backend "s3" {
    bucket         = "biglnotation-bucket-tfstate"
    key            = "terraform.tfstate"
    region         = "ap-southeast-2"
    dynamodb_table = "biglnotation-terraform-state"
  }
}

provider "aws" {
  region = "ap-southeast-2"
}
