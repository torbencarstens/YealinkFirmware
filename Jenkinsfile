#!groovy

properties([
        pipelineTriggers([cron('0 0 * * *')])
]
)

pipeline {
    agent any

    stages {
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
                    withCredentials([[$class: 'UsernamePasswordMultiBinding', credentialsId: 'aws-s3-credentials', usernameVariable: 'AWS_ACCESS_KEY_ID', passwordVariable: 'AWS_SECRET_ACCESS_KEY']]) {
                        def files = findFiles(glob: "*.rom")
                        def file = files[0]
                        sh "python3 deploy.py ${file}"
                    }
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
