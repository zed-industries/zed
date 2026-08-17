/*
    Copyright (C) 2010 Rob Buis <rwlbuis@gmail.com>
    Copyright (C) 2011 Cosmin Truta <ctruta@gmail.com>
    Copyright (C) 2012 University of Szeged
    Copyright (C) 2012 Renata Hodovan <reni@webkit.org>

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Library General Public
    License as published by the Free Software Foundation; either
    version 2 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Library General Public License for more details.

    You should have received a copy of the GNU Library General Public License
    along with this library; see the file COPYING.LIB.  If not, write to
    the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
    Boston, MA 02110-1301, USA.
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RESOURCE_DOCUMENT_CONTENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RESOURCE_DOCUMENT_CONTENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_status.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace base {
class SingleThreadTaskRunner;
}  // namespace base

namespace blink {

class AgentGroupScheduler;
class Document;
class FetchParameters;
class IsolatedSVGDocumentHost;
class KURL;
class SVGResourceDocumentObserver;

struct SVGResourceTarget;

// Representation of an SVG resource document. Fed from an SVGDocumentResource
// that update loading status and provide the document text content. Thus the
// "complex" made up of these two classes manage the load cycle for the content
// document. The load cycle of the complete content document can differ from
// that of the underlying resource if the content document itself has (data
// URL) subresources.
//
// Calling SVGResourceDocumentContent::Fetch() - the expected way of creating an
// SVGResourceDocumentContent - will return an instance that has its lifetime
// managed by the SVGResourceDocumentCache. The cache is responsible for
// disposing the instance when it is unused. The criteria for "is unused" is
// that no observers are registered with the SVGResourceDocumentContent
// instance. _If_ an instance is created directly, Dispose() _must_ be called
// before dropping the reference to the instance.
class CORE_EXPORT SVGResourceDocumentContent final
    : public GarbageCollected<SVGResourceDocumentContent> {
 public:
  static SVGResourceDocumentContent* Fetch(FetchParameters&, Document&);

  SVGResourceDocumentContent(AgentGroupScheduler&,
                             scoped_refptr<base::SingleThreadTaskRunner>);
  ~SVGResourceDocumentContent();

  bool IsLoaded() const;
  bool IsLoading() const;
  bool ErrorOccurred() const;

  Document* GetDocument() const;

  void NotifyStartLoad();

  enum class UpdateResult {
    kCompleted,
    kAsync,
    kError,
  };
  // Update the contained document using the text data in `content`, using
  // `request_url` as the document URL. Returns `kAsync` if the document's
  // 'load' event has not been dispatched.
  UpdateResult UpdateDocument(scoped_refptr<SharedBuffer> data,
                              const KURL& request_url);
  void ClearDocument();
  void Dispose();

  ResourceStatus GetStatus() const { return status_; }
  void UpdateStatus(ResourceStatus new_status);

  const KURL& Url() const;

  bool HasObservers() const { return !observers_.empty(); }
  void AddObserver(SVGResourceDocumentObserver*);
  void RemoveObserver(SVGResourceDocumentObserver*);
  void NotifyObservers();

  SVGResourceTarget* GetResourceTarget(const AtomicString& element_id);
  SVGResourceTarget* GetResourceTargetForRoot() const;
  void Trace(Visitor*) const;

 private:
  void NotifyObserver(SVGResourceDocumentObserver*);
  void ContentChanged();
  void LoadingFinished();
  void AsyncLoadingFinished();

  class ChromeClient;

  Member<IsolatedSVGDocumentHost> document_host_;
  Member<AgentGroupScheduler> agent_group_scheduler_;
  HeapHashSet<WeakMember<SVGResourceDocumentObserver>> observers_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  KURL url_;
  ResourceStatus status_ = ResourceStatus::kNotStarted;
  bool was_disposed_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RESOURCE_DOCUMENT_CONTENT_H_
