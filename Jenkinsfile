#!groovy

pipeline {
	agent any

	stages {
		stage('Build') {
			steps {
				echo '$PATH'
				sh 'cargo build'
			}
		}
	}
}
