#!groovy

pipeline {
	agent any

	stages {
		stage('Build stable') {
			steps {
				sh 'rustup default stable'
				sh 'cargo build'
			}
		}
		stage ('Build beta') {
            steps {
                sh 'rustup default beta'
				sh 'cargo build'
            }
		}
		stage ('Build nightly') {
            steps {
                sh 'rustup default nightly'
				sh 'cargo build'
            }
		}
        stage('Run stable') {
            sh 'rustup default stable'
            sh 'cargo run'
        }
        stage ('Test stable') {
            sh 'cargo test'
        }
        stage('Release') {
            sh 'cargo build --release'
        }
        stage('Archive') {
            when {
                expression {
                    fileExists '*.rom'
                }
            } steps {
                archiveArtifacts '*.rom'
            }
        }
	}
}
