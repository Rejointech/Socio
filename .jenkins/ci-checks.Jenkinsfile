pipeline {
  agent {
    label 'agent'
  }

  environment {
    ORG_NAME = 'prajjwalkumar17'
    VAS_REPO_NAME = 'Socio-jenkins'
    VAS_REPO_URL = "https://github.com/${ORG_NAME}/${VAS_REPO_NAME}"
    VAS_BRANCH = 'ci/webhook_tests'
  }

  stages {
    stage('Checkout VAS Main Branch') {
      steps {
        script {
          if(!github_event_head_commit_id)
            env.VAS_COMMIT = getBranchCommit()
          else
            env.VAS_COMMIT = github_event_head_commit_id
          checkoutRepository(env.ORG_NAME, env.VAS_REPO_NAME, env.VAS_BRANCH)
        }
      }
    }
    stage('CI-Checks') {
      failFast false
      parallel {
        stage('Spell Check') {
          steps {
            script {
              spellCheck()
            }
          }
          post {
            success {
              script {
                setVasSpellCheck('success')
              }
            }
            failure {
              script {
                setVasSpellCheck('failure')
              }
            }
          }
        }

        stage('Formatting Check') {
          steps {
            script {
              formattingCheck()
            }
          }
          post {
            success {
              script {
                setVasFormattingCheck('success')
              }
            }
            failure {
              script {
                setVasFormattingCheck('failure')
              }
            }
          }
        }
    }
  }
    stage('Compilation Checks') {
      failFast false
      parallel {
        stage('Test on Stable toolchain') {
          steps {
            script {
              setToolchain("stable")
              makeClippy()
              cargoHack()
            }
          }
          post {
            success {
              script {
                setVasStableCheck('success')
              }
            }
            failure {
              script {
                setVasStableCheck('failure')
              }
            }
          }
        }
      }
    }
  }
}

def setAllChecksInProgress() {
  env.CHECK_START_TIME = getCurrentTimestampIso8601()
  setVasSpellCheck('in_progress')
  setVasFormattingCheck('in_progress')
  setVasStableCheck('in_progress')
}

def setToolchain(toolchain) {
  sh "rustup default $toolchain"
}

def makeClippy() {
  sh 'cargo clippy --all-features --all-targets -- -D warnings'
}

def cargoHack() {
  sh 'cargo hack check --workspace --each-feature --all-targets'
}

def spellCheck() {
  sh 'typos'
}

def formattingCheck() {
  sh 'cargo +nightly fmt --all --check'
}

def getBranchCommit() {
  def branchResponse
    withCredentials(
      [usernamePassword(credentialsId: 'intruder-bot', passwordVariable: 'GH_TOKEN', usernameVariable: '425218')]
    ) {
    branchResponse = sh(
      script: """
        gh api \
          -H "Accept: application/vnd.github+json" \
          -H "X-GitHub-Api-Version: 2022-11-28" \
          /repos/${ORG_NAME}/${VAS_REPO_NAME}/branches/${env.VAS_BRANCH}
      """,
      returnStdout: true
    ).trim()
  }
return readJSON(text: branchResponse).commit.sha
}

def setVasSpellCheck(String commitStatus) {
  def checkName = 'Spell check'
  def detailsUrl = env.BUILD_URL
  def outputTitle
  def outputSummary
  def outputText = "Check the logs for more information (build number: ${BUILD_NUMBER})"

  if (commitStatus == 'success') {
    outputTitle = "Spell checks passed"
    outputSummary = "Spell checks passed"
  } else if (commitStatus == 'failure') {
    outputTitle = "Spell checks failed"
    outputSummary = "Spell checks failed"
  } else if (commitStatus == 'in_progress') {
    outputTitle = "Spell Checks in progress"
    outputSummary = "Checks are in progress"
    return setChecksInProgress(
      env.ORG_NAME,
      env.VAS_REPO_NAME,
      env.VAS_COMMIT,
      detailsUrl,
      commitStatus,
      checkName,
      outputTitle,
      outputSummary,
      outputText,
      env.CHECK_START_TIME ?: getCurrentTimestampIso8601(),
    )
  }

  setPrCheckCompleted(
    env.ORG_NAME,
    env.VAS_REPO_NAME,
    env.VAS_COMMIT,
    detailsUrl,
    commitStatus,
    checkName,
    outputTitle,
    outputSummary,
    outputText,
    env.CHECK_START_TIME ?: getCurrentTimestampIso8601(),
    env.CHECK_END_TIME ?: getCurrentTimestampIso8601()
  )
}

def setVasFormattingCheck(String commitStatus) {
  def checkName = 'Check formatting'
  def detailsUrl = env.BUILD_URL
  def outputTitle
  def outputSummary
  def outputText = "Check the logs for more information (build number: ${BUILD_NUMBER})"

  if (commitStatus == 'success') {
    outputTitle = "Formatting checks passed"
    outputSummary = "Formatting checks passed"
  } else if (commitStatus == 'failure') {
    outputTitle = "Formatting checks failed"
    outputSummary = "Formatting checks failed"
  } else if (commitStatus == 'in_progress') {
    outputTitle = "Formatting Checks in progress"
    outputSummary = "Checks are in progress"
    return setChecksInProgress(
      env.ORG_NAME,
      env.VAS_REPO_NAME,
      env.VAS_COMMIT,
      detailsUrl,
      commitStatus,
      checkName,
      outputTitle,
      outputSummary,
      outputText,
      env.CHECK_START_TIME ?: getCurrentTimestampIso8601(),
    )
  }

  setPrCheckCompleted(
    env.ORG_NAME,
    env. VAS_REPO_NAME,
    env.VAS_COMMIT,
    detailsUrl,
    commitStatus,
    checkName,
    outputTitle,
    outputSummary,
    outputText,
    env.CHECK_START_TIME ?: getCurrentTimestampIso8601(),
    env.CHECK_END_TIME ?: getCurrentTimestampIso8601()
  )
}

def setVasStableCheck(String commitStatus) {
  def checkName = 'Run tests on stable toolchain'
  def detailsUrl = env.BUILD_URL
  def outputTitle
  def outputSummary
  def outputText = "Check the logs for more information (build number: ${BUILD_NUMBER})"

  if (commitStatus == 'success') {
    outputTitle = "Compilation on Stable toolchain passed"
    outputSummary = "Compilation on Stable toolchain passed"
  } else if (commitStatus == 'failure') {
    outputTitle = "Compilation on Stable toolchain failed"
    outputSummary = "Compilation on Stable toolchain failed"
  } else if (commitStatus == 'in_progress') {
    outputTitle = "Stable toolchain compilation in progress"
    outputSummary = "Checks are in progress"
    return setChecksInProgress(
      env.ORG_NAME,
      env.VAS_REPO_NAME,
      env.VAS_COMMIT,
      detailsUrl,
      commitStatus,
      checkName,
      outputTitle,
      outputSummary,
      outputText,
      env.CHECK_START_TIME ?: getCurrentTimestampIso8601(),
    )
  }

  setPrCheckCompleted(
    env.ORG_NAME,
    env.VAS_REPO_NAME,
    env.VAS_COMMIT,
    detailsUrl,
    commitStatus,
    checkName,
    outputTitle,
    outputSummary,
    outputText,
    env.CHECK_START_TIME ?: getCurrentTimestampIso8601(),
    env.CHECK_END_TIME ?: getCurrentTimestampIso8601()
  )
}

def checkoutRepository(String orgName, String repoName, String branchName) {
  checkout changelog: false,
    poll: false,
    scm: scmGit(
      branches: [[
        name: "refs/heads/${branchName}"
      ]],
      extensions: [
        submodule(
          disableSubmodules: false,
          parentCredentials: true,
          recursiveSubmodules: true
        ),
        localBranch(branchName)
      ],
      userRemoteConfigs: [[
        credentialsId: 'hyperswitch-cloud-github-deploy-key',
        url: "git@github.com:${orgName}/${repoName}.git"
      ]]
    )
}

// Splits the string on a delimiter defined as: `zero or more whitespace, a literal comma, zero or more whitespace`
// which will place the words into the list and collapse any whitespace between the words and commas.
def splitCommaSeparatedStringAndTrim(String s) {
  new ArrayList<String>(Arrays.asList(s.split("\\s*,\\s*")))
}

// Configure git username and email
def configureGitUsernameAndEmail() {
  sh '''
    git config --local user.name 'hyperswitch-bot[bot]'
    git config --local user.email '148525504+hyperswitch-bot[bot]@users.noreply.github.com'
  '''
}

// Get current timestamp of UTC
def getCurrentTimestampIso8601() {
  new Date().format("yyyy-MM-dd'T'HH:mm:ss'Z'", TimeZone.getTimeZone("UTC"))
}

// Send Bot update to Github from Jenkins
def setChecksInProgress (
  String orgName,
  String repoName,
  String commitHash,
  String detailsUrl,
  String commitStatus,
  String checkName,
  String outputTitle,
  String outputSummary,
  String outputText,
  String checkStartTime
) {
  withCredentials(
    [usernamePassword(credentialsId: 'hyperswitch-bot-github-app', passwordVariable: 'GH_TOKEN', usernameVariable: 'GITHUB_APP_ID')]
  ) {
    sh """
      echo '{
        "name": "${checkName}",
        "head_sha": "${commitHash}",
        "details_url": "${detailsUrl}",
        "external_id": "${BUILD_NUMBER}",
        "status": "in_progress",
        "started_at": "${checkStartTime}",
        "output": {
          "title": "${outputTitle}",
          "summary": "${outputSummary}",
          "text": "${outputText}",
          "annotations": [],
          "images": []
        },
        "actions": []
      }' \
      | gh api \
        --method POST \
        --header "Accept: application/vnd.github+json" \
        --header "X-GitHub-Api-Version: 2022-11-28" \
        --input - \
        https://api.github.com/repos/${orgName}/${repoName}/check-runs
    """
  }

}

def setPrCheckCompleted (
  String orgName,
  String repoName,
  String commitHash,
  String detailsUrl,
  String commitStatus,
  String checkName,
  String outputTitle,
  String outputSummary,
  String outputText,
  String checkStartTime,
  String checkEndTime
) {
  // Conclusion can be one of: `action_required`, `cancelled`, `failure`, `neutral`, `success`, `skipped`, `stale`, `timed_out`
  withCredentials(
    [usernamePassword(credentialsId: 'hyperswitch-bot-github-app', passwordVariable: 'GH_TOKEN', usernameVariable: 'GITHUB_APP_ID')]
  ) {
    sh """
      echo '{
        "name": "${checkName}",
        "head_sha": "${commitHash}",
        "details_url": "${detailsUrl}",
        "external_id": "${BUILD_NUMBER}",
        "status": "completed",
        "started_at": "${checkStartTime}",
        "conclusion": "${commitStatus}",
        "completed_at": "${checkEndTime}",
        "output": {
          "title": "${outputTitle}",
          "summary": "${outputSummary}",
          "text": "${outputText}",
          "annotations": [],
          "images": []
        },
        "actions": []
      }' \
      | gh api \
        --method POST \
        --header "Accept: application/vnd.github+json" \
        --header "X-GitHub-Api-Version: 2022-11-28" \
        --input - \
        https://api.github.com/repos/${orgName}/${repoName}/check-runs
    """
  }
}

