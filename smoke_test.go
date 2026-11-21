package smoke

import "testing"

func TestReady(t *testing.T) {
	if !Ready() {
		t.Fatal("fixture is not ready")
	}
}
