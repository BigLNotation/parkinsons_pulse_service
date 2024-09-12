resource "aws_vpc" "vpc" {
  cidr_block           = "10.0.0.0/16"
  instance_tenancy     = "default"
  enable_dns_hostnames = true
}

resource "aws_subnet" "sn1" {
  cidr_block        = "10.0.1.0/24"
  vpc_id            = aws_vpc.vpc.id
  availability_zone = "ap-southeast-2a"
}

resource "aws_subnet" "sn2" {
  cidr_block        = "10.0.2.0/24"
  vpc_id            = aws_vpc.vpc.id
  availability_zone = "ap-southeast-2b"
}

resource "aws_vpc_endpoint" "ecr-dkr-endpoint" {
  vpc_id              = aws_vpc.vpc.id
  private_dns_enabled = true
  service_name        = "com.amazonaws.ap-southeast-2.ecr.dkr"
  vpc_endpoint_type   = "Interface"
  security_group_ids = [aws_security_group.sg.id]
  subnet_ids = [aws_subnet.sn1.id, aws_subnet.sn2.id]
}

resource "aws_vpc_endpoint" "ecr-api-endpoint" {
  vpc_id              = aws_vpc.vpc.id
  service_name        = "com.amazonaws.ap-southeast-2.ecr.api"
  vpc_endpoint_type   = "Interface"
  private_dns_enabled = true
  security_group_ids = [aws_security_group.sg.id]
  subnet_ids = [aws_subnet.sn1.id, aws_subnet.sn2.id]
}

resource "aws_security_group" "sg" {
  name   = "biglnotation_sg"
  vpc_id = aws_vpc.vpc.id

  ingress {
    description = "HTTPS"
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    description = "HTTP"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port = 0
    to_port   = 0
    protocol  = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}