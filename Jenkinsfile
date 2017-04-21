#!groovy

pipeline {
    agent any

    stages {
        stage('Build stable') {
            steps {
		script {
			def files = findFiles("*.rom")
			echo "${files}"
		}
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
        stage('Test') {
            steps {
                sh 'ls -aR'
		echo findFiles("*.rom")
            }
        }
        stage('Archive') {
            when {
                expression {
                    return !findFiles(glob: '*.rom').isEmpty()
                }
            }
            steps {
                archiveArtifacts '*.rom'
            }
        }
    }
    post {
        always {
            deleteDir()
        }
    }
}
