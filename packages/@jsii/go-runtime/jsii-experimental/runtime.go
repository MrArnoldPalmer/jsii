// Package jsii provides an interface for jsii generated go modules to interface with the jsii kernel process.
package jsii

import (
	"sync"
)

// runtime is a wrapper around a client instance that provides public methods
// for interacting with the client process. These methods handle client
// initialization and guarantee that libraries are all accessing the same client
// instance and can't accidentally create a duplicate process.
type runtime struct {
	client *client
}

var rtinstance *runtime
var once sync.Once

// getRuntime handles creation of the runtime instance if it hasn't been
// initialized.
func getRuntime() *runtime {
	once.Do(func() {
		client, err := initClient()

		if err != nil {
			panic(err)
		}

		rtinstance = &runtime{
			client: &client,
		}
	})

	return rtinstance
}

// rtRequest is the generic request/response function for the runtime instance.
func rtRequest(req KernelRequest, res KernelResponse) error {
	rt := getRuntime()
	return rt.client.request(req, res)
}

// Load gets called by each jsii module that is required before proxy objects
// can be instantiated within the kernel process. Module's should call Load once
// and only once.
func Load(request LoadRequest) (LoadResponse, error) {
	response := LoadResponse{}
	err := rtRequest(request, &response)
	return response, err
}

// Create will construct a new JSII object within the kernel runtime. This is
// called by jsii object constructors.
func Create(request CreateRequest) (CreateResponse, error) {
	response := CreateResponse{}
	err := rtRequest(request, &response)
	return response, err
}

// Get is used to access a property on a JSII object within the kernel runtime.
// This is called within getter methods.
func Get(request GetRequest) (GetResponse, error) {
	response := GetResponse{}
	err := rtRequest(request, &response)
	return response, err
}

// Invoke is used to call methods of JSII classes. It is called within the body
// of pointer receiver methods.
func Invoke(request InvokeRequest) (InvokeResponse, error) {
	response := InvokeResponse{}
	err := rtRequest(request, &response)
	return response, err
}

// StaticInvoke is used to invoke a class static method within the kernel
// process.
func StaticInvoke(request StaticInvokeRequest) (InvokeResponse, error) {
	response := InvokeResponse{}
	err := rtRequest(request, &response)
	return response, err
}
