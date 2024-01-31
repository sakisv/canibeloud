terraform {
  required_version = "~> 1.7.1"
  backend "s3" {
    bucket = "ooceenohm7-terraform-states"
    key    = "canibeloud/tfstate/production"
    region = "eu-west-2"
  }

  required_providers {
    aws = {
      version = "~> 5.34"
      source  = "hashicorp/aws"
    }

    hcloud = {
      source  = "hetznercloud/hcloud"
      version = "~> 1.45"
    }

    pass = {
      source  = "mecodia/pass"
      version = "~> 3.1"
    }

    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.23"
    }
  }
}
