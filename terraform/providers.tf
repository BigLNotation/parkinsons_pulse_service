terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5"
    }
  }

  backend "s3" {
    bucket         = "biglnotation-s3bucket-tfstate"
    key            = "pp_service.tfstate"
    region         = "ap-southeast-2"
    dynamodb_table = "biglnotation-dyndb-locktfstate"
  }
}

provider "aws" {
  region = "ap-southeast-2"
}
