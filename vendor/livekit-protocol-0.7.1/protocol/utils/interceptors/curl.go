package interceptors

import (
	"bytes"
	"context"
	"io"
	"net/http"
	"sort"

	"github.com/twitchtv/twirp"
	"google.golang.org/protobuf/encoding/protojson"
	"google.golang.org/protobuf/proto"
)

// NewCurlPrinter creates a Twirp interceptor that prints a curl commands for each request made.
func NewCurlPrinter(w io.Writer, url string) twirp.Interceptor {
	return func(next twirp.Method) twirp.Method {
		return func(ctx context.Context, req any) (any, error) {
			pkg, ok1 := twirp.PackageName(ctx)
			svc, ok2 := twirp.ServiceName(ctx)
			meth, ok3 := twirp.MethodName(ctx)
			if !ok1 || !ok2 || !ok3 {
				return next(ctx, req)
			}
			m, ok := req.(proto.Message)
			if !ok {
				return next(ctx, req)
			}
			hdr, _ := twirp.HTTPRequestHeaders(ctx)
			if err := printCurl(w, url, pkg, svc, meth, hdr, m); err != nil {
				return nil, err
			}
			return next(ctx, req)
		}
	}
}

func printCurl(w io.Writer, url, pkg, svc, meth string, hdr http.Header, req proto.Message) error {
	data, err := protojson.Marshal(req)
	if err != nil {
		return err
	}
	buf, isBuf := w.(*bytes.Buffer)
	if !isBuf {
		buf = new(bytes.Buffer)
	}
	buf.WriteString("curl ")
	buf.WriteString(`-X POST `)
	buf.WriteString("\\\n\t")
	hkeys := make([]string, 0, len(hdr))
	for k := range hdr {
		hkeys = append(hkeys, k)
	}
	sort.Strings(hkeys)
	for _, h := range hkeys {
		switch h {
		case "Content-Type":
			continue
		}
		for _, v := range hdr[h] {
			buf.WriteString(`-H '`)
			buf.WriteString(h)
			buf.WriteString(`: `)
			buf.WriteString(v)
			buf.WriteString(`' `)
			buf.WriteString("\\\n\t")
		}
	}
	buf.WriteString(`-H 'Content-Type: application/json' `)
	buf.WriteString("\\\n\t")
	buf.WriteString(`--data '`)
	buf.Write(data)
	buf.WriteString(`' `)
	buf.WriteString("\\\n\t")
	buf.WriteString(url)
	buf.WriteString("/twirp/")
	buf.WriteString(pkg)
	buf.WriteString(".")
	buf.WriteString(svc)
	buf.WriteString("/")
	buf.WriteString(meth)
	buf.WriteString("\n")
	if isBuf {
		return nil
	}
	_, err = buf.WriteTo(w)
	return err
}
