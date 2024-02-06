terraform {
  required_version = ">= 1.0.7"
  backend "s3" {
    bucket = "ooceenohm7-terraform-states"
    key    = "canibeloud/tfstate/ci"
    region = "eu-west-2"
  }

  required_providers {
    aws = {
      version = "~> 5.34"
      source  = "hashicorp/aws"
    }

    pass = {
      source  = "mecodia/pass"
      version = "~> 3.1"
    }
  }
}


provider "aws" {
  region = "eu-west-2"
}
