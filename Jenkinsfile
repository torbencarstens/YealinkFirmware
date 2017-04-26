#!groovy

pipeline {
    agent any

    stages {
        stage('Credential test') {
            steps {
                withCredentials([[$class: 'UsernamePasswordMultiBinding', credentialsId: 'aws-s3-credentials', usernameVariable: 'access_key_id', passwordVariable: 'secret_access_key']]) {
                    echo "${env.access_key_id}"
                    echo "${secret_access_key}"
                }
            }
        }
        stage('Run stable') {
            steps {
                sh 'rustup default stable'
                sh 'cargo run'
            }
        }
        stage('Archive') {
            when {
                expression {
                    def files = findFiles(glob: "*.rom")
                    return files != null && files.size() > 0
                }
            }
            steps {
                archiveArtifacts '*.rom'
            }
        }
        stage('Deploy') {
            when {
                expression {
                    def files = findFiles(glob: "*.rom")
                    return files != null && files.size() > 0
                }
            }
            steps {
                script {
                    def files = findFiles(glob: "*.rom")
                    def file = files[0]
                    sh "python3 deploy.py ${file}"
                }
            }
        }
    }
    post {
        always {
            deleteDir()
        }
    }
}
