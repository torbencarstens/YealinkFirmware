#!groovy

pipeline {
	agent any

	stages {
		stage('Build') {
			steps {
				sh 'echo $PATH'
				sh 'cargo build'
			}
		}
	}
}
