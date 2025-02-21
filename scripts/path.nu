

def path [] {
	# Get only the two most down paths
	$env.PWD | path split | last 2 | path join
}