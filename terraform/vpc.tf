resource "aws_vpc" "vpc" {
  cidr_block       = "10.0.0.0/16"
  instance_tenancy = "default"

  enable_dns_support   = true
  enable_dns_hostnames = true
}

resource "aws_subnet" "sn1_public" {
  cidr_block              = "10.0.1.0/24"
  vpc_id                  = aws_vpc.vpc.id
  availability_zone       = "ap-southeast-2a"
  map_public_ip_on_launch = true

  tags = {
    Name = "sn1_public"
  }
}

resource "aws_subnet" "sn2_public" {
  cidr_block              = "10.0.2.0/24"
  vpc_id                  = aws_vpc.vpc.id
  availability_zone       = "ap-southeast-2b"
  map_public_ip_on_launch = true

  tags = {
    Name = "sn2_public"
  }
}

resource "aws_subnet" "sn1_private" {
  cidr_block        = "10.0.3.0/24"
  vpc_id            = aws_vpc.vpc.id
  availability_zone = "ap-southeast-2a"

  tags = {
    Name = "sn1_private"
  }
}

resource "aws_subnet" "sn2_private" {
  cidr_block        = "10.0.4.0/24"
  vpc_id            = aws_vpc.vpc.id
  availability_zone = "ap-southeast-2b"

  tags = {
    Name = "sn2_private"
  }
}
