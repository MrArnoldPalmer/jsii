module github.com/aws/jsii

go 1.15

require (
	github.com/aws/jsii-runtime-go v0.0.0
	github.com/aws/jsii/jsii-calc/go/jsiicalc/v3 v3.20.120
	github.com/aws/jsii/jsii-calc/go/scopejsiicalclib v0.0.0
)

replace (
	github.com/aws/jsii-runtime-go v0.0.0 => ./jsii-runtime-go
	github.com/aws/jsii/jsii-calc/go/jsiicalc/v3 v3.20.120 => ./jsii-calc/go/jsiicalc
	github.com/aws/jsii/jsii-calc/go/scopejsiicalcbase v0.0.0 => ./jsii-calc/go/scopejsiicalcbase
	github.com/aws/jsii/jsii-calc/go/scopejsiicalcbaseofbase/v2 v2.1.1 => ./jsii-calc/go/scopejsiicalcbaseofbase
	github.com/aws/jsii/jsii-calc/go/scopejsiicalclib v0.0.0 => ./jsii-calc/go/scopejsiicalclib
)
