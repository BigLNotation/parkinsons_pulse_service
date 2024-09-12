resource "aws_ecr_repository" "pp_service_repo" {
  name                 = "biglnotation_pp_service_repo"
  image_tag_mutability = "MUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }
}
