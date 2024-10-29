resource "aws_internet_gateway" "internet_gw" {
  vpc_id = aws_vpc.vpc.id
}

resource "aws_eip" "nat" {
  domain = "vpc"
}

resource "aws_nat_gateway" "nat" {
  allocation_id = aws_eip.nat.id
  subnet_id     = aws_subnet.sn1_public.id
  depends_on    = [aws_internet_gateway.internet_gw]
}

## Public routing
resource "aws_route_table" "public" {
  vpc_id = aws_vpc.vpc.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.internet_gw.id
  }
}

resource "aws_route_table_association" "sn1_public" {
  route_table_id = aws_route_table.public.id
  subnet_id      = aws_subnet.sn1_public.id
}

resource "aws_route_table_association" "sn2_public" {
  route_table_id = aws_route_table.public.id
  subnet_id      = aws_subnet.sn2_public.id
}

## Private routing
resource "aws_route_table" "private" {
  vpc_id = aws_vpc.vpc.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.internet_gw.id
  }
}

resource "aws_route_table_association" "sn1_private" {
  route_table_id = aws_route_table.private.id
  subnet_id      = aws_subnet.sn1_private.id
}

resource "aws_route_table_association" "sn2_private" {
  route_table_id = aws_route_table.private.id
  subnet_id      = aws_subnet.sn2_private.id
}