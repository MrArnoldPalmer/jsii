package jsii

import (
	"bufio"
	"encoding/json"
	"errors"
	"github.com/aws-cdk/jsii/assets"
	"io"
	"io/ioutil"
	"os"
	"os/exec"
	"regexp"
	"time"
)

// client is the data structure responsible for intializing and managing the
// JSII kernel process. It has handles for writing and reading JSON to/from
// the processes STDIN and STDOUT.
type client struct {
	Process        *exec.Cmd
	RuntimeVersion string
	writer         *json.Encoder
	reader         *json.Decoder
	stderr         io.ReadCloser
}

// initClient starts the kernel child process and verifies that the runtime has
// intialized properly.
func initClient() (client, error) {
	clientinstance := client{}
	tmpfile, err := ioutil.TempFile("", "jsii-runtime.*.js")
	if err != nil {
		return clientinstance, err
	}

	defer os.Remove(tmpfile.Name())

	if _, err := tmpfile.Write([]byte(assets.Tarball)); err != nil {
		return clientinstance, err
	}

	if err := tmpfile.Close(); err != nil {
		return clientinstance, err
	}

	cmd := exec.Command("node", "--require", tmpfile.Name())

	out, err := cmd.StdoutPipe()
	if err != nil {
		return clientinstance, err
	}

	in, err := cmd.StdinPipe()
	if err != nil {
		return clientinstance, err
	}

	stderr, err := cmd.StderrPipe()
	if err != nil {
		return clientinstance, err
	}

	// Start Process
	if err := cmd.Start(); err != nil {
		return clientinstance, err
	}

	writer := json.NewEncoder(in)
	reader := json.NewDecoder(out)

	clientinstance = client{
		Process: cmd,
		writer:  writer,
		reader:  reader,
		stderr:  stderr,
	}

	// Check for OK response and parse runtime version
	rtver, err := clientinstance.validateClientStart()

	if err != nil {
		return client{}, err
	}

	clientinstance.RuntimeVersion = rtver
	return clientinstance, nil
}

// request accepts a KernelRequest struct which is encoded into a JSON string
// and written to the kernel processess' STDIN. It also accepts a pointer to a
// struct that is used for the output.
func (c *client) request(req KernelRequest, res KernelResponse) error {
	err := c.writer.Encode(req)
	if err != nil {
		return err
	}

	return c.response(res)
}

// response attempts to read a json value from the kernel processess' STDOUT and
// decode it into the passed response struct. If no value is found on STDOUT and
// STDERR has content, it will read the output of STDERR and return an error
// with that content as the error message.
func (c *client) response(res KernelResponse) error {
	// TODO: identify source of this race condition
	// Runtime locks without this timeout currently
	time.Sleep(time.Millisecond * 100)
	if c.reader.More() {
		return c.reader.Decode(res)
	}

	errrdr := bufio.NewReader(c.stderr)
	if errrdr.Size() > 0 {
		erroutput, err := ioutil.ReadAll(errrdr)

		if err != nil {
			return err
		}

		return errors.New(string(erroutput))
	}

	return errors.New("No Response from runtime")

}

// validateClientStart verifies that the expected response is written to the
// process STDOUT after initialization. It parses the version of the kernel
// runtime returned on the initial output.
func (c *client) validateClientStart() (string, error) {
	response := InitOkResponse{}

	if err := c.response(&response); err != nil {
		return "", err
	}

	version := regexp.MustCompile("@").Split(response.Hello, 3)[2]
	return version, nil
}
