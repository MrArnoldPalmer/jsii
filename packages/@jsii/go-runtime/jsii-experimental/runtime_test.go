package jsii

import (
	"encoding/json"
	"fmt"
	"testing"
)

func TestClient(t *testing.T) {
	client, err := initClient()
	if err != nil {
		t.Log(err)
		t.Errorf(fmt.Sprintf("Client init error: %s", err.Error()))
	}

	if client.RuntimeVersion == "" {
		clientstr, _ := json.Marshal(&client)
		t.Errorf("No client runtime version found\nClient: %s", string(clientstr))
	}
}
