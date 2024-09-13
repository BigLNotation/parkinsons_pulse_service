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
}

resource "aws_subnet" "sn2_public" {
  cidr_block              = "10.0.2.0/24"
  vpc_id                  = aws_vpc.vpc.id
  availability_zone       = "ap-southeast-2b"
  map_public_ip_on_launch = true
}

resource "aws_subnet" "sn1_private" {
  cidr_block        = "10.0.3.0/24"
  vpc_id            = aws_vpc.vpc.id
  availability_zone = "ap-southeast-2a"
}

resource "aws_subnet" "sn2_private" {
  cidr_block        = "10.0.4.0/24"
  vpc_id            = aws_vpc.vpc.id
  availability_zone = "ap-southeast-2b"
}

resource "aws_internet_gateway" "internet_gw" {
  vpc_id = aws_vpc.vpc.id
}

resource "aws_eip" "nat" {
  domain = "vpc"
}

resource "aws_nat_gateway" "nat" {
  allocation_id = aws_eip.nat.id
  subnet_id = aws_subnet.sn1_public.id
  depends_on = [aws_internet_gateway.internet_gw]
}

# resource "aws_security_group" "sg" {
#   name   = "biglnotation_sg"
#   vpc_id = aws_vpc.vpc.id
#
#   ingress {
#     description = "Helloworld test"
#     from_port = 5678
#     to_port = 5678
#     protocol = "tcp"
#     cidr_blocks = ["0.0.0.0/0"]
#   }
#
#   ingress {
#     description = "HTTPS"
#     from_port   = 443
#     to_port     = 443
#     protocol    = "tcp"
#     cidr_blocks = ["0.0.0.0/0"]
#   }
#
#   ingress {
#     description = "HTTP"
#     from_port   = 80
#     to_port     = 80
#     protocol    = "tcp"
#     cidr_blocks = ["0.0.0.0/0"]
#   }
#
#   egress {
#     from_port = 0
#     to_port   = 0
#     protocol  = "-1"
#     cidr_blocks = ["0.0.0.0/0"]
#   }
# }

resource "aws_route_table" "private" {
  vpc_id = aws_vpc.vpc.id

  route {
    cidr_block = "0.0.0.0/0"
    nat_gateway_id = aws_nat_gateway.nat.id
  }
}

resource "aws_route_table" "public" {
  vpc_id = aws_vpc.vpc.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.internet_gw.id
  }
}

resource "aws_route_table_association" "sn1_private" {
  route_table_id = aws_route_table.private.id
  subnet_id = aws_subnet.sn1_private.id
}

resource "aws_route_table_association" "sn2_private" {
  route_table_id = aws_route_table.private.id
  subnet_id = aws_subnet.sn2_private.id
}

resource "aws_route_table_association" "sn1_public" {
  route_table_id = aws_route_table.public.id
  subnet_id = aws_subnet.sn1_public.id
}

resource "aws_route_table_association" "sn2_public" {
  route_table_id = aws_route_table.public.id
  subnet_id = aws_subnet.sn2_public.id
}