output "ecs_cluster_name" {
  value = aws_ecs_cluster.pp_service_cluster.name
}

output "ecs_service_name" {
  value = aws_ecs_service.pp_service_service.name
}

output "ecr_repository_name" {
  value = aws_ecr_repository.pp_service_repo.name
}

output "ecr_registry_url" {
  value = aws_ecr_repository.pp_service_repo.repository_url
}

output "alb_hostname" {
  value = aws_alb.pp_service.dns_name
}

output "deployed_hostname" {
  value = aws_route53_record.api_alias.records
}
