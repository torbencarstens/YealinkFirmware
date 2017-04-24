#!groovy

pipeline {
    agent any

    stages {
        stage('Build stable') {
            steps {
                sh 'rustup default stable'
                sh 'cargo build -v'
            }
        }
        stage('Build beta') {
            steps {
                sh 'rustup default beta'
                sh 'cargo build -v'
            }
        }
        stage('Build nightly') {
            steps {
                sh 'rustup default nightly'
                sh 'cargo build -v'
            }
        }
        stage('Run stable') {
            steps {
                sh 'rustup default stable'
                sh 'cargo run'
            }
        }
        stage('Test stable') {
            steps {
                sh 'cargo test'
            }
        }
        stage('Release') {
            steps {
                sh 'cargo build --release'
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
                def files = findFiles(glob: "*.rom")
                sh 'python3 deploy.py ${files[0]}'
            }
        }
    }
    post {
        always {
            deleteDir()
        }
    }
}
