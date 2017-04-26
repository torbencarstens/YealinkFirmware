#!groovy

pipeline {
    agent any

    stages {
        stage('Credential test') {
            steps {
                withCredentials([[$class: 'FileBinding', credentialsId: '{876876-9687-ABFE}', variable: 'SECRET_FILE']]) {
                    echo "${SECRET_FILE}"
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
